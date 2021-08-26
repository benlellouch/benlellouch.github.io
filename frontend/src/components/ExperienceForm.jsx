import React from 'react'
import Button from './Button'
import './Form.css'
import { useState } from 'react'

// {
//     id:1,
//     title:"Software Engineer",
//     company:"J.P. Morgan",
//     year: "jun 2021 - aug 2021",
//     description:"This is my first website built with React",
//     org_link:"https://github.com/benlellouch/benlellouch.github.io"
//   }

export const ExperienceForm = ({onClose, onAdd}) => {
    const [title, setTitle] = useState('')
    const [company, setCompany] = useState('')
    const [year, setYear] = useState('')
    const [description, setDescription] = useState('')
    const [org_link, setOrglink] = useState('')
  
    const onSubmit = (e) => {
      e.preventDefault()
  
      if (!description) {
        alert('Please add a task')
        return
      }
  
      onAdd({ title, company, year, description, org_link })
  
      setTitle('')
      setOrglink('')
      setDescription('')
      setYear('')
      setCompany('')
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
                <label>Company</label>
                <input
                    type='text'
                    placeholder='Add Company'
                    value={company}
                    onChange={(e) => setCompany(e.target.value)}
                />
                </div>
                <div className='form-control'>
                <label>Year</label>
                <input
                    type='text'
                    placeholder='Add Year'
                    value={year}
                    onChange={(e) => setYear(e.target.value)}
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
                <label>Organisation Link</label>
                <input
                    type='text'
                    placeholder='Add URL'
                    value={org_link}
                    onChange={(e) => setOrglink(e.target.value)}
                />
                </div>
        
                <input type='submit' value='Save Task' className='btn btn-block' />
            </form>
        </div>

    )
}
