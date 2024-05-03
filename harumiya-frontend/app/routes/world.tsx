import { defer, redirect, useLoaderData, useActionData, useParams, unstable_useViewTransitionState, json, useNavigation, Await } from "@remix-run/react";
import type { LoaderFunction, ActionFunction } from '@remix-run/node';
import Section from "~/components/section";
import fantasy from "/fantasy.webp";
import world from "/world.jpg";
import React from "react";


interface WorldData {
  data: {
    [key: string]: string;
  };
}


const action: ActionFunction = async ({ request }) => {
  const formData = await request.formData();
  let keys = Array.from(formData.keys()).length

  for (let [key, val] of formData.entries()) {
    console.log(key, val);
  }

  const entries = Object.fromEntries(formData.entries());
  let defaultRoute = 'http://localhost:8080/create'

  if (keys > 1) {
    defaultRoute = 'http://localhost:8080/create-detailed'
  }


  const res = await fetch(defaultRoute, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ premise: entries }),
    keepalive: true
  });

  const json = await res.text();

  try {
    return defer(JSON.parse(json));
  } catch (error) {
    console.error("Error parsing JSON:", error);
    return redirect(`/world?error=${encodeURIComponent("Failed to parse JSON")}`);
  }


}

const loader: LoaderFunction = async ({ params }) => {
  return params;
}

export { action, loader };


export default function WorldOverview() {
  const navigation = useNavigation();

  const worldData = useActionData() as WorldData;

  console.log("TESTING", worldData);

  if (worldData === undefined) {
    return <div style={{
      backgroundImage: `url(${fantasy})`,
      backgroundSize: "cover",
      height: "100vh", // Set the height of the div to cover the entire page
      width: "100vw", // Set the width of the div to cover the entire page
      position: "relative", // Add position relative to the div
      overflow: "hidden",
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
      padding: 0

    }}> <h1 style={{
      textAlign: "center",
      color: "green",
      position: "relative", // Add position relative to the text
      zIndex: 1, // Set a higher z-index to make the text appear above the overlay
    }}>Loading...</h1>
    </div>
  }

  return (
    <div style={{
      backgroundImage: `url(${world})`,
      backgroundSize: "cover",
      height: "100vh", // Set the height of the div to cover the entire page
      width: "100vw", // Set the width of the div to cover the entire page
      position: "relative",
    }}>
      <React.Suspense fallback={<div>Loading...</div>}>
        <Await resolve={worldData} >
          {(worldData: WorldData) => (
            <pre>
              {Object.entries(worldData.data).map(([key, value]) => (
                <Section key={key} title={key} content={value} />
              ))}
            </pre>
          )}
        </Await>
      </React.Suspense>
    </div >
  );

}
