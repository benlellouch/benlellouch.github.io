import React from 'react'
import Button from './Button'

const Card = ({ item, onDelete, loggedIn, type }) => {
  const renderContent = () => {
    if (type === 'project') {
      return (
        <>
          <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2 transition-colors duration-300">
            <a 
              href={item.link} 
              className="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 transition-colors"
              target="_blank"
              rel="noopener noreferrer"
            >
              {item.title}
            </a>
            {loggedIn && (
              <span className="ml-3">
                <Button 
                  color="red" 
                  text="Delete" 
                  onClick={() => onDelete(item.id)} 
                />
              </span>
            )}
          </h3>
          <p className="text-gray-700 dark:text-gray-300 leading-relaxed transition-colors duration-300">{item.description}</p>
        </>
      )
    }
    
    if (type === 'experience') {
      return (
        <>
          <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2 transition-colors duration-300">
            {item.title} - 
            <a 
              href={item.org_link}
              className="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 transition-colors ml-1"
              target="_blank"
              rel="noopener noreferrer"
            >
              {item.company}
            </a>
            <span className="text-gray-600 dark:text-gray-400 font-normal transition-colors duration-300"> ({item.year})</span>
            {loggedIn && (
              <span className="ml-3">
                <Button 
                  color="red" 
                  text="Delete" 
                  onClick={() => onDelete(item.id)} 
                />
              </span>
            )}
          </h3>
          <p className="text-gray-700 dark:text-gray-300 leading-relaxed transition-colors duration-300">{item.description}</p>
        </>
      )
    }
    
    return null
  }

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-4 hover:shadow-lg transition-all duration-300">
      {renderContent()}
    </div>
  )
}

export default Card