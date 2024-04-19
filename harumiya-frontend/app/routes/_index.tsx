import type { MetaFunction } from "@remix-run/node";
import { Form, redirect, useActionData } from "@remix-run/react";
import type { ActionFunctionArgs } from "@remix-run/node";
import { commitSession, getSession } from "../sessions";

export const meta: MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Harumiya!" },
  ];
};

export const action = async ({
  request
}: ActionFunctionArgs) => {
  // console.log("FIRING initial");

  // const session = await getSession(
  //   request.headers.get("Cookie")
  // );
  // const formData = await request.formData();
  // const premise = formData.get("premise");

  // session.flash(
  //   "error",
  //   `${premise}`
  // );


  return redirect("/world")

}


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
    </div>
  );
}
