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
            <div className="section" style={{ textAlign: 'center' }}>
                <h1 style={{ fontSize: '4em', color: 'goldenrod' }}>{content.toUpperCase()}</h1>
            </div>
        );
    }

    return (
        <div className="section" style={{ textAlign: 'center' }}>
            <h1 style={{ color: 'goldenrod' }}>{title.toUpperCase()}</h1>
            <p style={{
                color: 'goldenrod',
                wordBreak: 'break-word',
                whiteSpace: 'normal',
                position: "relative", // Add position relative to the text
                zIndex: 1,
            }}>{content}</p>
        </div >
    );
};

export default Section;