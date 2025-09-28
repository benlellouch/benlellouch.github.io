import React from 'react'
import Card from './Card'

const ItemList = ({ items, onDelete, loggedIn, type, title, onAdd }) => {
  if (!items || items.length === 0) {
    return (
      <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-8 text-center transition-colors duration-300">
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-4 flex items-center justify-center transition-colors duration-300">
          {title}
          {loggedIn && onAdd && (
            <button
              onClick={onAdd}
              className="ml-4 bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-md transition-colors text-sm font-medium"
            >
              Add
            </button>
          )}
        </h2>
        <p className="text-gray-600 dark:text-gray-400 transition-colors duration-300">No {type}s to display yet.</p>
      </div>
    )
  }

  return (
    <div className="mb-8">
      <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-6 flex items-center justify-between transition-colors duration-300">
        {title}
        {loggedIn && onAdd && (
          <button
            onClick={onAdd}
            className="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-md transition-colors text-sm font-medium"
          >
            Add
          </button>
        )}
      </h2>
      <div className="space-y-4">
        {items.map((item) => (
          <Card
            key={item.id}
            item={item}
            onDelete={onDelete}
            loggedIn={loggedIn}
            type={type}
          />
        ))}
      </div>
    </div>
  )
}

export default ItemList