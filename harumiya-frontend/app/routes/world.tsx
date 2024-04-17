import { redirect, useLoaderData, useParams } from "@remix-run/react";
import type { LoaderFunction, ActionFunction } from '@remix-run/node';

const action: ActionFunction = async ({ request }) => {
  console.log("FIRING ACTION");
  const formData = await request.formData();
  console.log("USER", formData.get("premise"));

  const res = await fetch("http://localhost:8000/create", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ premise: formData.get("premise") }),
  });
  const text = await res.text();
  console.log("TEXT", text);

  return JSON.parse(text);
}

export { action };

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
