export default (req, context) => {
  console.log(context.params);
  return context.params;
}
