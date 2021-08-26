import React from 'react'
import Button from './Button'

export const Experience = ({experience, onDelete, loggedIn}) => {
    return (
        <div className="entry">
            <h3>{experience.title} - <a href={experience.org_link}>{experience.company}</a> ( {experience.year} )
             {loggedIn ? <Button color={'red'} text={'Delete'} onClick={() => onDelete(experience.id )} /> : null}
              </h3>
            <p>{experience.description}</p>
            
        </div>
    )
}
