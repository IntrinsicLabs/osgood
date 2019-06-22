
const failedResponse = new Response("Failed", {status: 500});

export default async (req, ctx) => {

  const withQuery = await fetch("http://localhost:9001/");
  if (withQuery.status !== 200){
    return failedResponse;
  }

  const withHash = await fetch("http://localhost:9001/#hello");
  if (withHash.status !== 200){
    return failedResponse;
  }

  const withBoth = await fetch("http://localhost:9001/?query=test");
  if (withBoth.status !== 200){
    return failedResponse;
  }

  try {
    const wrongURL = await fetch("http://localho:9001/");
    return failedResponse;
  } catch (e) {}

  try {
    const wrongURL_2 = await fetch("http://localhos?t:9001");
    return failedResponse;
  } catch (e) {}

  try {
    const wrongURL_3 = await fetch("http://localhos#t:9001");
    return failedResponse;
  } catch (e) {}

  return "success";
}

