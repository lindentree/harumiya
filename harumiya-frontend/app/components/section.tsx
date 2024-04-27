import React from 'react';

interface SectionProps {
    title: string;
    content: string;
}

const Section: React.FC<SectionProps> = ({ title, content }) => {
    if (!title || !content) {
        return null;
    }


    if (title === 'name') {
        return (
            <div className="section">
                <h1>{content.toUpperCase()}</h1>
            </div>
        );
    }

    return (
        <div className="section">
            <h1>{title.toUpperCase()}</h1>
            <p>{content}</p>
        </div>
    );
};

export default Section;