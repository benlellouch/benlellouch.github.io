const API_BASE_URL = 'http://localhost:8000'

// Generic API request function
const apiRequest = async (endpoint, options = {}) => {
  const url = `${API_BASE_URL}${endpoint}`
  const config = {
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
      ...options.headers
    },
    ...options
  }

  try {
    const response = await fetch(url, config)
    
    // Handle different response types
    if (response.status === 204 || response.status === 200) {
      // For DELETE operations or empty responses
      if (response.status === 204) {
        return { success: true, data: null, status: response.status }
      }
      
      // Try to parse JSON, but handle empty responses
      let data = null
      try {
        const text = await response.text()
        data = text ? JSON.parse(text) : null
      } catch (parseError) {
        // If JSON parsing fails but status is OK, treat as success
        console.warn('JSON parse failed but request succeeded:', parseError)
        return { success: true, data: null, status: response.status }
      }
      
      return { success: true, data, status: response.status }
    }
    
    // Handle error responses
    if (!response.ok) {
      let errorData = null
      try {
        errorData = await response.json()
      } catch (parseError) {
        // If we can't parse error response, use status text
        return { success: false, error: response.statusText || 'Request failed', status: response.status }
      }
      
      return { success: false, error: errorData.message || errorData.error || 'Request failed', status: response.status }
    }
    
    // This shouldn't happen, but just in case
    const data = await response.json()
    return { success: true, data, status: response.status }
    
  } catch (error) {
    console.error('API request failed:', error)
    return { success: false, error: error.message || 'Network error', status: null }
  }
}

// Generic CRUD operations
export const apiService = {
  // Get all items from a resource
  getAll: async (resource) => {
    return await apiRequest(`/${resource}`)
  },

  // Create a new item
  create: async (resource, data) => {
    return await apiRequest(`/${resource}`, {
      method: 'POST',
      body: JSON.stringify(data)
    })
  },

  // Delete an item by ID
  delete: async (resource, id) => {
    return await apiRequest(`/${resource}/${id}`, {
      method: 'DELETE'
    })
  },

  // Update an item by ID
  update: async (resource, id, data) => {
    return await apiRequest(`/${resource}/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data)
    })
  },

  // Auth-specific endpoints
  auth: {
    login: async (credentials) => {
      return await apiRequest('/login', {
        method: 'POST',
        body: JSON.stringify(credentials)
      })
    },

    logout: async () => {
      return await apiRequest('/logout', {
        method: 'POST'
      })
    },

    checkAuth: async () => {
      return await apiRequest('/login')
    }
  }
}