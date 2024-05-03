import React, { useState } from 'react';

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

    // const handleInputChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    //     const { name, value } = e.target;
    //     setWorldDetails((prevDetails) => ({
    //         ...prevDetails,
    //         [name]: value,
    //     }));
    // };

    // Add more properties as needed
    const handleInputChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const { name, value } = e.target;
        setWorldDetails((prevDetails) => ({
            ...prevDetails,
            [name]: value,
        }));
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        // Handle form submission here
        console.log(worldDetails);
    };

    return (
        <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
            <form onSubmit={handleSubmit}>
                <input name="premise" type="text" style={{ height: "80px", borderRadius: "10px", paddingLeft: "10px" }} placeholder="Enter premise here" />
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