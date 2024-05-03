import { useNavigation } from "@remix-run/react";
import { useTransition } from "react";

import DetailedPrompt from "~/components/detailedPrompt";

import scifi from "/scifi.jpg";
import fantasy from "/fantasy.webp";


const DetailPage = () => {
    const navigation = useNavigation();
    const [isPending, startTransition] = useTransition();

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
            backgroundImage: `url(${scifi})`,
            backgroundSize: "cover",
            height: "100vh", // Set the height of the div to cover the entire page
            width: "100vw", // Set the width of the div to cover the entire page
            overflow: "hidden"

        }}>
            <h1 style={{
                textAlign: "center",
                color: "green",
            }}>Detailed Prompt Form</h1>
            <DetailedPrompt />
        </div>
    );
};

export default DetailPage;