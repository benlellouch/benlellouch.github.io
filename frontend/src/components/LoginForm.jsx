import React from 'react'
import Button from './Button'
import './Form.css'
import { useState } from 'react'

export const LoginForm = ({onClose, onAdd}) => {

    const [username, setUsername] = useState('')
    const [password, setPassword] = useState('')
  
    const onSubmit = (e) => {
      e.preventDefault()
  
      if (!password) {
        alert('Please add a task')
        return
      }
  
      onAdd({ username, password})
  
      setUsername('')
      setPassword('')
    }

    return (
        <div>
            <div> 
            <Button color={"red"} text={"x"} onClick={onClose}/>
            <form className='add-form' onSubmit={onSubmit}>
                <div className='form-control'>
                <label>Username</label>
                <input
                    type='text'
                    placeholder='Enter Username'
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                />
                </div>
            
                <div className='form-control'>
                <label>Password</label>
                <input
                    type='password'
                    placeholder='Enter Password'
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
                </div>
        
                <input type='submit' value='Login' className='btn btn-block' />
            </form>
        </div>
        </div>
    )
}
