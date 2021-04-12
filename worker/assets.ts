declare const SOLOMON_KV: KVNamespace;

export const getAssets = async (key: string): Promise<Response> => {
  const buffer = await SOLOMON_KV.get(key, "arrayBuffer");

  if (!buffer) throw new Error();

  let headers: HeadersInit;

  switch (key.split(".").pop()) {
    case "js":
      headers = {
        "content-type": "text/javascript; charset=utf-8",
        "cache-control": "public, max-age=31536000, immutable",
      };
      break;
    case "css":
      headers = {
        "content-type": "text/css; charset=utf-8",
        "cache-control": "public, max-age=31536000, immutable",
      };
      break;
    case "gpg":
      headers = {
        "content-type": "text/plain; charset=utf-8",
        "cache-control": "no-cache",
      };
      break;
    case "jpg":
    case "jpeg":
      headers = {
        "content-type": "image/jpeg",
        "cache-control": "public, max-age=31536000, immutable",
      };
      break;
    case "png":
      headers = {
        "content-type": "image/png",
        "cache-control": "public, max-age=31536000, immutable",
      };
      break;
    case "ico":
      headers = {
        "content-type": "image/x-icon",
        "cache-control": "public, max-age=31536000, immutable",
      };
      break;
    case "svg":
      headers = {
        "content-type": "image/svg+xml",
        "cache-control": "public, max-age=31536000, immutable",
      };
      break;
    default:
      headers = {
        "content-type": "application/octet-stream",
      };
  }

  return new Response(buffer, { status: 200, headers });
};
