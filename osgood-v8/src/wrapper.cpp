#include <libplatform/libplatform.h>
#include <v8-platform.h>
#include <v8.h>

#define V8_TYPES(V)                                                        \
  V(v8::Value, value)                                                          \
  V(v8::Object, object)                                                        \
  V(v8::Array, array)                                                          \
  V(v8::ArrayBuffer, array_buffer)                                             \
  V(v8::String, string)                                                        \
  V(v8::Number, number)                                                        \
  V(v8::Integer, integer)                                                      \
  V(v8::Boolean, boolean)                                                      \
  V(v8::Function, function)                                                    \
  V(v8::Context, context)                                                      \
  V(v8::Module, module)                                                        \
  V(v8::Message, message)                                                      \
  V(v8::Script, script)

#define EMPTY_MAYBE(TYPE, NAME)                                                \
  v8::MaybeLocal<TYPE> empty_##NAME() { return v8::MaybeLocal<TYPE>(); }
#define TO_PERSISTENT(TYPE, NAME)                                              \
  v8::Persistent<TYPE, v8::CopyablePersistentTraits<TYPE>> * persistent_from_##NAME(v8::Isolate * isolate, v8::Local<TYPE> local) {                                   \
    v8::Persistent<TYPE, v8::CopyablePersistentTraits<TYPE>> * ret = new v8::Persistent<TYPE, v8::CopyablePersistentTraits<TYPE>>(isolate, local); \
    return ret;                               \
  }
#define FROM_PERSISTENT(TYPE, NAME)                                              \
  v8::Local<TYPE> persistent_to_##NAME(v8::Isolate * isolate, v8::Persistent<TYPE> * persistent) {                                   \
    return v8::Local<TYPE>::New(isolate, *persistent); \
  }
#define PERSISTENT_RESET(TYPE, NAME)                                           \
  void persistent_reset_##NAME(v8::Persistent<TYPE> * persistent) {                 \
    persistent->Reset();                                                       \
  }

static std::unique_ptr<v8::Platform> g_platform;

namespace osgood {

struct RunJSResult {
  v8::Local<v8::Value> ret_val;
  bool is_exception;
};

struct RunJSModuleResult {
  v8::Local<v8::Module> ret_val;
  v8::Local<v8::Value> exception;
  bool is_exception;
};

struct CompileJSResult {
  v8::Local<v8::Script> ret_val;
  v8::Local<v8::Value> exception;
  bool is_exception;
};

void platform_init(const char *argv0, const char * flags, int flags_len) {
  v8::V8::InitializeICUDefaultLocation(argv0);
  v8::V8::InitializeExternalStartupData(argv0);
  g_platform = v8::platform::NewDefaultPlatform();
  if (flags_len > 0) {
    v8::V8::SetFlagsFromString(flags, flags_len);
  }
  v8::V8::InitializePlatform(g_platform.get());
  v8::V8::Initialize();
}

void platform_dispose() {
  v8::V8::Dispose();
  v8::V8::ShutdownPlatform();
}

void process_messages(v8::Isolate *isolate) {
  v8::SealHandleScope shs(isolate);
  while (v8::platform::PumpMessageLoop(g_platform.get(), isolate)) {
    isolate->RunMicrotasks();
  }
  if (g_platform->IdleTasksEnabled(isolate)) {
    v8::platform::RunIdleTasks(g_platform.get(), isolate,
                               1.0 / 1000 /* 1 millisecond */);
  }
}

v8::Local<v8::Context> new_context(v8::Isolate *isolate) {
  return v8::Context::New(isolate);
}

v8::Local<v8::FunctionTemplate> new_function_template(v8::Isolate *isolate,
                                                      v8::FunctionCallback fn) {
  return v8::FunctionTemplate::New(isolate, fn);
}

v8::Local<v8::Value>
info_get_arg(const v8::FunctionCallbackInfo<v8::Value> &info, int i) {
  return info[i];
}

v8::Isolate *info_get_isolate(const v8::FunctionCallbackInfo<v8::Value> &info) {
  return info.GetIsolate();
}

void info_set_return_value(const v8::FunctionCallbackInfo<v8::Value> &info,
                           v8::Local<v8::Value> value) {
  info.GetReturnValue().Set(value);
}

v8::Local<v8::Primitive> null(v8::Isolate *isolate) {
  return v8::Null(isolate);
}

RunJSResult run_script(v8::Isolate *isolate, v8::Local<v8::Context> context,
                       v8::Local<v8::Script> script) {
  v8::TryCatch try_catch(isolate);
  v8::MaybeLocal<v8::Value> ret = script->Run(context);
  if (try_catch.HasCaught()) {
    return RunJSResult{try_catch.Exception(), true};
  } else {
    return RunJSResult{ret.ToLocalChecked(), false};
  }
}

CompileJSResult compile_script(v8::Isolate *isolate, v8::Local<v8::Context> ctx,
                               v8::Local<v8::String> src) {
  v8::TryCatch try_catch(isolate);
  v8::MaybeLocal<v8::Script> maybe_script = v8::Script::Compile(ctx, src);
  CompileJSResult result;
  if (try_catch.HasCaught()) {
    result.exception = try_catch.Exception();
    result.is_exception = true;
  } else {
    result.ret_val = maybe_script.ToLocalChecked();
    result.is_exception = false;
  }
  return result;
}

RunJSModuleResult compile_module(v8::Isolate *isolate, v8::ScriptOrigin origin,
                                 v8::Local<v8::String> code) {
  v8::TryCatch try_catch(isolate);
  v8::ScriptCompiler::Source source(code, origin);
  v8::MaybeLocal<v8::Module> ret =
      v8::ScriptCompiler::CompileModule(isolate, &source);
  RunJSModuleResult result;
  if (try_catch.HasCaught()) {
    result.exception = try_catch.Exception();
    result.is_exception = true;
  } else {
    result.ret_val = ret.ToLocalChecked();
    result.is_exception = false;
  }
  return result;
}

bool instantiate_module(v8::Local<v8::Context> context,
                        v8::Local<v8::Module> module,
                        v8::Module::ResolveCallback callback) {
  v8::Maybe<bool> res = module->InstantiateModule(context, callback);
  if (res.IsJust()) {
    return res.FromJust();
  }

  return false;
}

RunJSResult evaluate_module(v8::Isolate *isolate,
                            v8::Local<v8::Context> context,
                            v8::Local<v8::Module> module) {
  v8::TryCatch try_catch(isolate);
  v8::MaybeLocal<v8::Value> ret = module->Evaluate(context);
  if (try_catch.HasCaught()) {
    return RunJSResult{try_catch.Exception(), true};
  } else {
    return RunJSResult{ret.ToLocalChecked(), false};
  }
}

v8::ScriptOrigin create_module_origin(v8::Isolate *isolate,
                                      v8::Local<v8::String> name) {
  return v8::ScriptOrigin(name, v8::Local<v8::Integer>(),
                          v8::Local<v8::Integer>(), v8::Local<v8::Boolean>(),
                          v8::Local<v8::Integer>(), v8::Local<v8::Value>(),
                          v8::Local<v8::Boolean>(), v8::Local<v8::Boolean>(),
                          v8::True(isolate));
}

int get_identity_hash(v8::Local<v8::Module> module) {
  return module->GetIdentityHash();
}

v8::MaybeLocal<v8::Module> from_local_module(v8::Local<v8::Module> module) {
  return v8::MaybeLocal<v8::Module>(module);
}

V8_TYPES(EMPTY_MAYBE)
V8_TYPES(TO_PERSISTENT)
V8_TYPES(FROM_PERSISTENT)
V8_TYPES(PERSISTENT_RESET)

} // namespace osgood
