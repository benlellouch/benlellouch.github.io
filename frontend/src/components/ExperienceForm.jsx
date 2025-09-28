import React, { useState } from 'react'
import Button from './Button'

export const ExperienceForm = ({ onClose, onAdd, error }) => {
  const [title, setTitle] = useState('')
  const [company, setCompany] = useState('')
  const [year, setYear] = useState('')
  const [description, setDescription] = useState('')
  const [org_link, setOrglink] = useState('')

  const onSubmit = (e) => {
    e.preventDefault()

    if (!title || !company || !description) {
      alert('Please fill in all required fields')
      return
    }

    onAdd({ 
      title, 
      company, 
      year, 
      description, 
      org_link,
      clearForm: () => {
        setTitle('')
        setOrglink('')
        setDescription('')
        setYear('')
        setCompany('')
        onClose()
      }
    })
  }

  return (
    <div className="w-full max-w-md">
      <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-6 transition-colors duration-300">Add Experience</h2>
      
      {error && (
        <div className="mb-4 p-3 bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-600 text-red-700 dark:text-red-400 rounded-md text-sm transition-colors duration-300">
          {error}
        </div>
      )}
      
      <form onSubmit={onSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Job Title *
          </label>
          <input
            type="text"
            placeholder="e.g. Software Engineer"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors duration-300"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Company *
          </label>
          <input
            type="text"
            placeholder="e.g. Google"
            value={company}
            onChange={(e) => setCompany(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors duration-300"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Year/Period
          </label>
          <input
            type="text"
            placeholder="e.g. Jan 2021 - Dec 2022"
            value={year}
            onChange={(e) => setYear(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors duration-300"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Description *
          </label>
          <textarea
            placeholder="Describe your role and achievements..."
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows="4"
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none transition-colors duration-300"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 transition-colors duration-300">
            Company Website
          </label>
          <input
            type="url"
            placeholder="https://company.com"
            value={org_link}
            onChange={(e) => setOrglink(e.target.value)}
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
            text="Save Experience"
            color="blue"
            onClick={onSubmit}
          />
        </div>
      </form>
    </div>
  )
}
