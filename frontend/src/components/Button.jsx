import React from 'react'
import PropTypes from 'prop-types'

const Button = ({ color = 'blue', text, onClick, size = 'md', variant = 'solid' }) => {
  const getColorClasses = () => {
    const colors = {
      blue: {
        solid: 'bg-blue-500 hover:bg-blue-600 text-white',
        outline: 'border-blue-500 text-blue-500 hover:bg-blue-50'
      },
      green: {
        solid: 'bg-green-500 hover:bg-green-600 text-white',
        outline: 'border-green-500 text-green-500 hover:bg-green-50'
      },
      red: {
        solid: 'bg-red-500 hover:bg-red-600 text-white',
        outline: 'border-red-500 text-red-500 hover:bg-red-50'
      },
      gray: {
        solid: 'bg-gray-500 hover:bg-gray-600 text-white',
        outline: 'border-gray-500 text-gray-500 hover:bg-gray-50'
      }
    }
    return colors[color]?.[variant] || colors.blue[variant]
  }

  const getSizeClasses = () => {
    const sizes = {
      sm: 'px-3 py-1.5 text-sm',
      md: 'px-4 py-2 text-sm',
      lg: 'px-6 py-3 text-base'
    }
    return sizes[size] || sizes.md
  }

  const baseClasses = 'font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500'
  const variantClasses = variant === 'outline' ? 'border-2 bg-transparent' : ''

  return (
    <button
      onClick={onClick}
      className={`${baseClasses} ${variantClasses} ${getColorClasses()} ${getSizeClasses()}`}
    >
      {text}
    </button>
  )
}

Button.propTypes = {
  text: PropTypes.string.isRequired,
  color: PropTypes.oneOf(['blue', 'green', 'red', 'gray']),
  onClick: PropTypes.func.isRequired,
  size: PropTypes.oneOf(['sm', 'md', 'lg']),
  variant: PropTypes.oneOf(['solid', 'outline'])
}

export default Button