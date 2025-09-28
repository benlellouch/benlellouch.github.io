import React, { useState } from 'react'
import Button from './Button'

export const ProjectForm = ({ onClose, onAdd, error }) => {
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [link, setLink] = useState('')

  const onSubmit = (e) => {
    e.preventDefault()

    if (!title || !description) {
      alert('Please fill in all required fields')
      return
    }

    onAdd({ 
      title, 
      description, 
      link,
      clearForm: () => {
        setTitle('')
        setLink('')
        setDescription('')
        onClose()
      }
    })
  }

  return (
    <div className="w-full max-w-md">
      <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-6 transition-colors duration-300">Add Project</h2>
      
      {error && (
        <div className="mb-4 p-3 bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-600 text-red-700 dark:text-red-400 rounded-md text-sm transition-colors duration-300">
          {error}
        </div>
      )}
      
      <form onSubmit={onSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Project Title *
          </label>
          <input
            type="text"
            placeholder="e.g. Personal Portfolio Website"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors duration-300"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Description *
          </label>
          <textarea
            placeholder="Describe your project, technologies used, and key features..."
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows="4"
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none transition-colors duration-300"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Project URL
          </label>
          <input
            type="url"
            placeholder="https://github.com/username/project"
            value={link}
            onChange={(e) => setLink(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors duration-300"
          />
        </div>

        <div className="flex gap-3 pt-4">
          <Button
            text="Cancel"
            color="gray"
            variant="outline"
            onClick={onClose}
          />
          <Button
            text="Save Project"
            color="blue"
            onClick={onSubmit}
          />
        </div>
      </form>
    </div>
  )
}
