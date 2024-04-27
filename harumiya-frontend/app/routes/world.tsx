import { redirect, useLoaderData, useActionData, useParams, json } from "@remix-run/react";
import type { LoaderFunction, ActionFunction } from '@remix-run/node';
import { ReactNode } from "react";
import Section from "~/components/section";

const action: ActionFunction = async ({ request }) => {
  console.log("FIRING ACTION");
  const formData = await request.formData();

  const res = await fetch('http://localhost:8080/create', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ premise: formData.get('premise') }),
    keepalive: true
  });

  //const external = await res.text();
  const json = await res.text();

  //const valid = JSON.stringify(json);
  console.log("EXTERNAL", json);
  return JSON.parse(json);

}

const loader: LoaderFunction = async ({ params }) => {
  console.log("params", params);
  return params;
}

export { action, loader };


export default function WorldOverview() {
  const worldData = useActionData() as JSON;

  if (!worldData) {
    return <div>Loading...</div>;
  }
  //const actualWorldData = JSON.parse(worldData as string);
  console.log("TESTING", worldData);
  console.log("WORLD", Object.keys(worldData));
  //const actualWorldData = JSON.parse(JSON.stringify(worldData, null, 2));
  //console.log("ACTUAL", JSON.parse(actualWorldData));
  return (
    <div>
      <h1>World Overview</h1>
      <div>
        <pre>
          {Object.entries(worldData).map(([key, value]) => (
            <Section key={key} title={key} content={value} />
          ))}
        </pre>
      </div>
    </div>
  );

}
