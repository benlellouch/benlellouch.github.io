import React from 'react'
import Project from './Project'

const Projects = ({projects}) => {
    return (
        <>
            {projects.map( (project) => (
                        <Project key={project.id} project={project}/>
                    )
                )
            }
        </>
    )
}

export default Projects;
