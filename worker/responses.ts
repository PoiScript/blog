export const html = (body: string) =>
  new Response(body, {
    status: 200,
    headers: {
      "content-type": "text/html; charset=utf-8",
    },
  });

export const rss = (body: string) =>
  new Response(body, {
    status: 200,
    headers: {
      "content-type": "application/xml; charset=utf-8",
    },
  });

export const text = (body: string) =>
  new Response(body, {
    status: 200,
    headers: {
      "content-type": "text/plain; charset=utf-8",
    },
  });

export const redirect = (location: string) =>
  new Response(null, { status: 301, headers: { location } });
