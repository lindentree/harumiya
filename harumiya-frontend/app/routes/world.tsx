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
    return defer(JSON.parse(json));
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
  const navigation = useNavigation();

  const worldData = useActionData() as WorldData | undefined;

  if (!worldData) {
    return <div>Loading...</div>;
  }
  console.log("TESTING", worldData);
  console.log("WORLD", Object.keys(worldData));

  if (navigation.state === "loading") {
    return <div style={{
      backgroundImage: `url(${fantasy})`,
      backgroundSize: "cover",
      height: "100vh", // Set the height of the div to cover the entire page
      width: "100vw", // Set the width of the div to cover the entire page
      position: "relative", // Add position relative to the div

    }}>Loading...</div>;
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
