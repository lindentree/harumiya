import type { MetaFunction } from "@remix-run/node";
import { Form, redirect, useActionData, useNavigate, useNavigation } from "@remix-run/react";
import type { ActionFunctionArgs } from "@remix-run/node";
import cover from "/Library-Fantasy-World.jpg";
import fantasy from "/fantasy.webp";
import { useTransition } from "react";

export const meta: MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Harumiya!" },
  ];
};

export const action = async ({
  request
}: ActionFunctionArgs) => {
  return redirect("/world")
}

export default function Index() {
  const navigation = useNavigation();
  const [isPending, startTransition] = useTransition();
  const navigate = useNavigate();

  if (navigation.state === "loading" || navigation.state !== "idle" || isPending) {
    return (
      <div style={{
        backgroundImage: `url(${fantasy})`,
        backgroundSize: "cover",
        height: "100vh", // Set the height of the div to cover the entire page
        width: "100vw", // Set the width of the div to cover the entire page
        position: "relative", // Add position relative to the div
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}>
        <h1 style={{
          color: "green",
          position: "relative", // Add position relative to the text
          zIndex: 1, // Set a higher z-index to make the text appear above the overlay
        }}>Generating...</h1>
      </div>
    );
  }


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
      position: "relative", // Add position relative to the div
    }}>
      <div style={{
        position: "absolute", // Add position absolute to the overlay
        top: 0,
        left: 0,
        width: "100%",
        height: "100%",
        backgroundColor: "rgba(0, 0, 0, 0.5)", // Add a transparent black background color
      }}></div>
      <h1 style={{
        color: "green",
        position: "relative", // Add position relative to the text
        zIndex: 1, // Set a higher z-index to make the text appear above the overlay
      }}>Welcome to Harumiya!</h1>
      <p style={{
        textAlign: "center",
        color: "green",
        position: "relative", // Add position relative to the text
        zIndex: 1, // Set a higher z-index to make the text appear above the overlay
      }}> Harumiya is an idea generator for world building</p>
      <div>
        <Form method="post" action="/world">
          <input name="premise" type="text" style={{
            height: "200px", width: "600px", borderRadius: "10px", paddingLeft: "10px", wordBreak: 'break-word',
            whiteSpace: 'normal',
            zIndex: 1,
            position: "relative"
          }} placeholder="Enter premise here" />
          <br />
          <button type="submit" style={{ fontSize: "1.2em", position: "relative", zIndex: 1 }}>Demo</button>
        </Form>
      </div>

      <p style={{
        color: "green",
        textAlign: "center",
        position: "relative", // Add position relative to the text
        zIndex: 1, // Set a higher z-index to make the text appear above the overlay
      }}>If you have a more involved premise that can't be summed up in one or two sentences, please try our detailed prompt form.</p>
      <button type="button" onClick={() => navigate("/detail")} style={{ position: "relative", zIndex: 1 }}>Go to Detailed Prompt Form</button>
    </div >
  );
}
