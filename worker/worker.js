addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);

  try {
    event.respondWith(handleRequest(url.pathname));
  } catch (e) {
    event.respondWith(new Response(e.stack, { status: 500 }));
  }
});

async function handleRequest(url) {
  /* WASM_BINDGEN_SCRIPT */

  await wasm_bindgen(wasm);
  return wasm_bindgen.handleRequest(url);
}
