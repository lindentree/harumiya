import type { MetaFunction } from "@remix-run/node";
import { Form, useActionData } from "@remix-run/react";
import type { ActionFunctionArgs } from "@remix-run/node";
import WorldOverview from "./world";

export const meta: MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Harumiya!" },
  ];
};

// export const action = async ({
//   request
// }: ActionFunctionArgs) => {
//   console.log("FIRING ACTION");

//   const res = await fetch("http://localhost:8000/create", {
//     method: "POST",
//     headers: {
//       "Content-Type": "application/json",
//     },
//     body: JSON.stringify({ premise: request.formData() }),
//   });

//   return res.json();
// }


export default function Index() {

  return (
    <div style={{ fontFamily: "system-ui, sans-serif", lineHeight: "1.8" }}>
      <h1>Welcome to Harumiya!</h1>
      <div>
        <Form method="post" action="/world">
          <input name="premise" type="text" />
          <button type="submit">Submit</button>
        </Form>
      </div>
      <WorldOverview />
    </div>
  );
}
