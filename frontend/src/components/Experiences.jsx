import React from 'react'
import { Experience } from './Experience';

const Experiences = ({experiences, onDelete, loggedIn}) => {
    return (
        <>
            {experiences.map( (experience) => (
                        <Experience key={experience.id} experience={experience} onDelete={onDelete} loggedIn={loggedIn}/>
                    )
                )
            }
        </>
    )
}

export default Experiences;
