import DetailedPrompt from "~/components/detailedPrompt";

import scifi from "/scifi.jpg";


const DetailPage = () => {
    return (
        <div style={{
            backgroundImage: `url(${scifi})`,
            backgroundSize: "cover",
            height: "100vh", // Set the height of the div to cover the entire page
            width: "100vw", // Set the width of the div to cover the entire page
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