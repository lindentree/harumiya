import { redirect, useLoaderData, useActionData, useParams, unstable_useViewTransitionState, json } from "@remix-run/react";
import type { LoaderFunction, ActionFunction } from '@remix-run/node';
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
  try {
    return JSON.parse(json);
  } catch (error) {
    console.error("Error parsing JSON:", error);
    return redirect(`/world?error=${encodeURIComponent("Failed to parse JSON")}`);
  }


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
  console.log("TESTING", worldData);
  console.log("WORLD", Object.keys(worldData));

  return (
    <div>
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
