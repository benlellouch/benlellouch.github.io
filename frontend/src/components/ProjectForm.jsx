import React from 'react'
import Button from './Button'
import './Form.css'
import { useState } from 'react'

// {
//     id:1,
//     title:"This website",
//     description:"This is my first website built with React",
//     link:"https://github.com/benlellouch/benlellouch.github.io"
//   },

export const ProjectForm = ({onClose, onAdd}) => {
    const [title, setTitle] = useState('')
    const [description, setDescription] = useState('')
    const [link, setLink] = useState('')
  
    const onSubmit = (e) => {
      e.preventDefault()
  
      if (!description) {
        alert('Please add a task')
        return
      }
  
      onAdd({ title, description, link })
  
      setTitle('')
      setLink('')
      setDescription('')
      onClose()
    }
  
    return (
        <div> 
            <Button color={"red"} text={"x"} onClick={onClose}/>
            <form className='add-form' onSubmit={onSubmit}>
                <div className='form-control'>
                <label>Job Title</label>
                <input
                    type='text'
                    placeholder='Add Job Title'
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                />
                </div>
            
                <div className='form-control'>
                <label>Description</label>
                <textarea
                    type='text'
                    placeholder='Add Description'
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                />
                </div>
                <div className='form-control'>
                <label>Project URL</label>
                <input
                    type='text'
                    placeholder='Add URL'
                    value={link}
                    onChange={(e) => setLink(e.target.value)}
                />
                </div>
        
                <input type='submit' value='Save Task' className='btn btn-block' />
            </form>
        </div>

    )
}
