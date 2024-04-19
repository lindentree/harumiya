import { redirect, useLoaderData, useActionData, useParams } from "@remix-run/react";
import type { LoaderFunction, ActionFunction } from '@remix-run/node';
import { getSession, commitSession } from "../sessions";

const action: ActionFunction = async ({ request }) => {
  console.log("FIRING ACTION");
  const formData = await request.formData();

  const res = await fetch("http://localhost:8000/create/sse", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ premise: formData.get("premise") }),
  });
  console.log("RES", res);
  const text = await res.text();
  console.log("ALL", text);
  const data = JSON.parse(text);
  console.log("JSON", data);
  console.log("FIRST", text[0]);

  return redirect(`/world/${data.id}`);

}

const loader: LoaderFunction = async ({ params }) => {
  return params;
}

export { action };
export { loader };

export default function WorldOverview() {
  const worldData = useLoaderData();
  console.log(worldData);
  return (
    <div>


      <h1>World Overview</h1>
      <div>
        <h2>World Data</h2>
        <pre>{JSON.stringify(worldData, null, 2)}</pre>
      </div>
    </div>

  );

}
