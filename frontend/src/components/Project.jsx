import React from 'react'

const Project = ({project}) => {
    return (
        <div>
            <h3><a href={project.link}> {project.title} </a> </h3>
            {/* <img src={project.img_path}/> */}
            <p>{project.description}</p>
        </div>
    )
}

export default Project;

