import React, { useState } from 'react';
import { useNavigation } from "@remix-run/react";
import { useTransition } from "react";

import fantasy from "/fantasy.webp";

interface WorldDetails {
    premise: string;
    genre: string;
    secondaryGenre: string;
    use: string;
    // Add more properties as needed
}


const DetailedPrompt: React.FC = () => {
    const [worldDetails, setWorldDetails] = useState<WorldDetails>({
        premise: '',
        genre: '',
        secondaryGenre: '',
        use: '',
        // Initialize other properties here
    });

    // Add more properties as needed
    const handleInputChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const { name, value } = e.target;
        setWorldDetails((prevDetails) => ({
            ...prevDetails,
            [name]: value,
        }));
    };

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
        <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
            <form method="post" action="/world">
                <input name="premise" type="text"
                    autoComplete="off"
                    style={{
                        height: "200px",
                        width: "600px",
                        borderRadius: "10px", paddingLeft: "10px", wordBreak: 'break-word',
                        whiteSpace: 'normal',
                    }} placeholder="Enter premise here" />
                <label style={{ display: "block", fontSize: "1.2rem", color: "green" }}>
                    Genre:
                    <select
                        name="genre"
                        value={worldDetails.genre}
                        onChange={(e: React.ChangeEvent<HTMLSelectElement>) => handleInputChange(e)}
                    >
                        <option value="fantasy">Fantasy</option>
                        <option value="scifi">Sci-Fi</option>
                    </select>
                </label>
                <label style={{ display: "block", fontSize: "1.2rem", color: "green" }}>
                    Secondary Genre:
                    <select
                        name="genre"
                        value={worldDetails.genre}
                        onChange={(e: React.ChangeEvent<HTMLSelectElement>) => handleInputChange(e)}
                    >
                        <option value="romance">Romance</option>
                        <option value="comedy">Comedy</option>
                    </select>
                </label>
                <label style={{ display: "block", fontSize: "1.2rem", color: "green" }}>
                    Intended Use:
                    <select
                        name="use"
                        value={worldDetails.use}
                        onChange={(e: React.ChangeEvent<HTMLSelectElement>) => handleInputChange(e)}
                    >
                        <option value="novel">Novel</option>
                        <option value="shortStory">Short Story</option>
                        <option value="rpg">RPG</option>
                        <option value="game">Video Game</option>
                    </select>
                </label>
                {/* Add more form fields here */}
                <button type="submit">Submit</button>
            </form>
        </div >
    );

};

export default DetailedPrompt;