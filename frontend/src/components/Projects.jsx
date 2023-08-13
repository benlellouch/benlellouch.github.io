import React from 'react'
import Project from './Project'

const Projects = ({projects, onDelete, loggedIn}) => {
    return (
        <>
            {projects.map( (project) => (
                        <Project key={project.id} project={project} onDelete={onDelete} loggedIn={loggedIn}/>
                    )
                )
            }
        </>
    )
}

export default Projects;
