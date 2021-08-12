import React from 'react'
import { Experience } from './Experience';

const Experiences = ({experiences}) => {
    return (
        <>
            {experiences.map( (experience) => (
                        <Experience key={experience.id} experience={experience}/>
                    )
                )
            }
        </>
    )
}

export default Experiences;
