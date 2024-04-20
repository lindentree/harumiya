import { redirect, useLoaderData, useActionData, useParams } from "@remix-run/react";
import type { LoaderFunction, ActionFunction } from '@remix-run/node';
import axios from 'axios';

const action: ActionFunction = async ({ request }) => {
  console.log("FIRING ACTION");
  const formData = await request.formData();

  // const res = await axios.post(
  //   "http://localhost:8000/create",
  //   { premise: formData.get("premise") },
  //   {
  //     headers: {
  //       "Content-Type": "application/json",
  //     }
  //   }
  // ).then
  //   (res => {
  //     const data = res.data
  //     console.log("RES", data);
  //     return redirect(`/world/${data.name}`);

  //   });

  const res = await fetch('http://localhost:8000/create', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ premise: formData.get('premise') }),
    keepalive: true
  });

  const external = await res.json();
  console.log("EXTERNAL", external);


}

const loader: LoaderFunction = async ({ params }) => {
  console.log("params", params);
  return params;
}

export { action, loader };


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
