const API = "http://localhost:1234/api";

export default async function query<T>(gql: TemplateStringsArray | string): Promise<T[]> {
  const result = await fetch(API, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      query: gql
    })
  });
  const json = await result.json();
  return json.data.myAlbums as T[];
}
