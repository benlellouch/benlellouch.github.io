import React from 'react'
import Button from './Button'

const Project = ({project, onDelete, loggedIn}) => {
    return (
        <div className="entry">
            <h3><a href={project.link}> {project.title} </a>
             {loggedIn ? <Button color={'red'} text={'Delete'} onClick={() => onDelete(project.id )} /> : null} 
              </h3>
            {/* <img src={project.img_path}/> */}
            <p>{project.description}</p>
        </div>
    )
}

export default Project;

