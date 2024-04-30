import type { MetaFunction } from "@remix-run/node";
import { Form, redirect, useActionData } from "@remix-run/react";
import type { ActionFunctionArgs } from "@remix-run/node";
import { commitSession, getSession } from "../sessions";
import cover from "/Library-Fantasy-World.jpg";

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
    <div style={{
      fontFamily: "system-ui, sans-serif",
      textAlign: "center",
      color: "green",
      lineHeight: "1.8",
      backgroundImage: `url(${cover})`,
      backgroundSize: "cover",
      height: "100vh", // Set the height of the div to cover the entire page
      width: "100vw", // Set the width of the div to cover the entire page
    }}>
      <h1 style={{
        color: "green"
      }}>Welcome to Harumiya!</h1>
      <p style={{
        textAlign: "center",
        color: "green",
      }}> Harumiya is an idea generator for world building</p>
      <div>
        <Form method="post" action="/world">
          <input name="premise" type="text" style={{ height: "80px", borderRadius: "10px", paddingLeft: "10px" }} placeholder="Enter premise here" />
          <br />
          <button type="submit" style={{ fontSize: "1.2em" }}>Demo</button>
        </Form>
      </div>
    </div >
  );
}
