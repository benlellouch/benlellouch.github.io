import React from 'react'

export const Experience = ({experience}) => {
    return (
        <div>
            <h3>{experience.title} - <a href={experience.org_link}>{experience.company}</a> ( {experience.year} )</h3>
            <p>{experience.description}</p>
        </div>
    )
}
