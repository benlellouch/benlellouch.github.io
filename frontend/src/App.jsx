import React, { useState, useEffect } from 'react'
import ItemList from './components/ItemList'
import Button from './components/Button'
import ModalForm from './components/ModalForm'
import DarkModeToggle from './components/DarkModeToggle'
import { ExperienceForm } from './components/ExperienceForm'
import { ProjectForm } from './components/ProjectForm'
import { LoginForm } from './components/LoginForm'
import { useDarkMode } from './hooks/useDarkMode'
import { apiService } from './utils/api'

function App() {
  const [projects, setProjects] = useState([])
  const [experiences, setExperiences] = useState([])
  const [showExperienceForm, setShowExperienceForm] = useState(false)
  const [showProjectForm, setShowProjectForm] = useState(false)
  const [showLoginForm, setShowLoginForm] = useState(false)
  const [loggedIn, setLoggedIn] = useState(false)
  const [loginError, setLoginError] = useState('')
  const [experienceError, setExperienceError] = useState('')
  const [projectError, setProjectError] = useState('')
  const { isDark, toggleDarkMode } = useDarkMode()

  useEffect(() => {
    const initializeApp = async () => {
      await Promise.all([
        loadProjects(),
        loadExperiences(),
        attemptAutoLogin()
      ])
    }

    initializeApp()
  }, [])

  // API Functions
  const loadProjects = async () => {
    try {
      const result = await apiService.getAll('projects')
      if (result.success) {
        setProjects(result.data)
      } else {
        console.error('Failed to load projects:', result.error)
      }
    } catch (error) {
      console.error('Failed to load projects:', error)
    }
  }

  const loadExperiences = async () => {
    try {
      const result = await apiService.getAll('experiences')
      if (result.success) {
        setExperiences(result.data)
      } else {
        console.error('Failed to load experiences:', result.error)
      }
    } catch (error) {
      console.error('Failed to load experiences:', error)
    }
  }

  const attemptAutoLogin = async () => {
    try {
      const result = await apiService.auth.checkAuth()
      if (result.success && result.data.success) {
        setLoggedIn(true)
      }
    } catch (error) {
      console.error('Auto-login failed:', error)
    }
  }

  // Experience handlers
  const deleteExperience = async (id) => {
    try {
      const result = await apiService.delete('experiences', id)
      console.log('Delete experience result:', result) // Debug logging
      
      if (result.success) {
        setExperiences(experiences.filter((exp) => exp.id !== id))
      } else {
        console.error('Delete failed:', result)
        alert(`Failed to delete experience: ${result.error}`)
      }
    } catch (error) {
      console.error('Delete error:', error)
      alert('Failed to delete experience')
    }
  }

  const addExperience = async (experienceData) => {
    try {
      setExperienceError('') // Clear any previous errors
      const { clearForm, ...experience } = experienceData
      
      const result = await apiService.create('experiences', experience)

      if (result.success) {
        setExperiences([...experiences, result.data])
        setExperienceError('')
        setShowExperienceForm(false)
        clearForm()
      } else {
        setExperienceError(result.error || 'Failed to add experience. Please try again.')
      }
    } catch (error) {
      setExperienceError('Failed to add experience. Please check your connection.')
    }
  }

  // Project handlers
  const deleteProject = async (id) => {
    try {
      const result = await apiService.delete('projects', id)
      console.log('Delete project result:', result) // Debug logging
      
      if (result.success) {
        setProjects(projects.filter((project) => project.id !== id))
      } else {
        console.error('Delete failed:', result)
        alert(`Failed to delete project: ${result.error}`)
      }
    } catch (error) {
      console.error('Delete error:', error)
      alert('Failed to delete project')
    }
  }

  const addProject = async (projectData) => {
    try {
      setProjectError('') // Clear any previous errors
      const { clearForm, ...project } = projectData
      
      const result = await apiService.create('projects', project)

      if (result.success) {
        setProjects([...projects, result.data])
        setProjectError('')
        setShowProjectForm(false)
        clearForm()
      } else {
        setProjectError(result.error || 'Failed to add project. Please try again.')
      }
    } catch (error) {
      setProjectError('Failed to add project. Please check your connection.')
    }
  }

  // Auth handlers
  const login = async (credentials) => {
    try {
      setLoginError('') // Clear any previous errors
      const result = await apiService.auth.login(credentials)

      if (result.success && result.data.success) {
        setLoggedIn(true)
        setShowLoginForm(false)
        setLoginError('')
      } else {
        setLoginError('Incorrect username or password')
      }
    } catch (error) {
      setLoginError('Login failed. Please try again.')
    }
  }

  const logout = async () => {
    try {
      const result = await apiService.auth.logout()
      console.log('Logout result:', result) // Debug logging

      if (result.success) {
        setLoggedIn(false)
      } else {
        console.error('Logout failed:', result)
        alert(`Failed to logout: ${result.error}`)
      }
    } catch (error) {
      console.error('Logout error:', error)
      alert('Failed to logout')
    }
  }

  const handleLoginClick = async () => {
    const result = await apiService.auth.checkAuth()
    if (result.success && result.data.success) {
      setLoggedIn(true)
    } else {
      setLoginError('') // Clear any previous errors when opening login form
      setShowLoginForm(true)
    }
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-300 flex flex-col">
      {/* Dark Mode Toggle */}
      <DarkModeToggle isDark={isDark} onToggle={toggleDarkMode} />

      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow-sm transition-colors duration-300">
        <div className="max-w-4xl mx-auto px-4 py-8">
          <div className="text-center">
            <img
              src="https://media.licdn.com/dms/image/v2/D4E03AQFdnuZAAnTQ_A/profile-displayphoto-shrink_400_400/profile-displayphoto-shrink_400_400/0/1663869957381?e=1768435200&v=beta&t=dPGTI6LtHeb-pVvTIOqDdzQw9gz3PdrxcbdDAHCHSno"
              alt="Benjamin Lellouch" 
              className="w-32 h-32 rounded-full mx-auto mb-4 object-cover shadow-lg"
            />
            <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors duration-300">Benjamin Lellouch</h1>
            
            {/* Social Links */}
            <div className="flex items-center justify-center gap-4 mb-6">
              <a
                href="https://linkedin.com/in/benjamin-lellouch"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 shadow-md hover:shadow-lg"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
                </svg>
                LinkedIn
              </a>
              
              <a
                href="https://github.com/benlellouch"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 px-4 py-2 bg-gray-800 hover:bg-gray-900 dark:bg-gray-700 dark:hover:bg-gray-600 text-white rounded-lg transition-colors duration-300 shadow-md hover:shadow-lg"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                GitHub
              </a>
            </div>
            
            <Button
              color={loggedIn ? "red" : "green"}
              text={loggedIn ? "Logout" : "Login"}
              onClick={loggedIn ? logout : handleLoginClick}
            />
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-4xl mx-auto px-4 py-8 space-y-12">
        <ItemList
          items={projects}
          onDelete={deleteProject}
          loggedIn={loggedIn}
          type="project"
          title="Projects"
          onAdd={() => { setProjectError(''); setShowProjectForm(true); }}
        />

        <ItemList
          items={experiences}
          onDelete={deleteExperience}
          loggedIn={loggedIn}
          type="experience"
          title="Experience"
          onAdd={() => { setExperienceError(''); setShowExperienceForm(true); }}
        />
      </main>

      {/* Footer */}
      <footer className="bg-white dark:bg-gray-800 shadow-sm transition-colors duration-300 mt-auto">
        <div className="max-w-4xl mx-auto px-4 py-6">
          <div className="text-center">
            <p className="text-gray-600 dark:text-gray-400 text-sm transition-colors duration-300 flex items-center justify-center gap-2 flex-wrap">
              Made with 
              <span className="text-red-500 text-base">❤️</span>
              using
              <span className="flex items-center gap-1 bg-orange-100 dark:bg-orange-900/20 px-2 py-1 rounded-md">
                <span className="text-base">🦀</span>
                Rust
              </span>
              and
              <span className="flex items-center gap-1 bg-blue-100 dark:bg-blue-900/20 px-2 py-1 rounded-md">
                <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none">
                  <circle cx="12" cy="12" r="1.5" fill="#61DAFB"/>
                  <ellipse cx="12" cy="12" rx="11" ry="4.2" stroke="#61DAFB" strokeWidth="1" fill="none"/>
                  <ellipse cx="12" cy="12" rx="11" ry="4.2" stroke="#61DAFB" strokeWidth="1" fill="none" transform="rotate(60 12 12)"/>
                  <ellipse cx="12" cy="12" rx="11" ry="4.2" stroke="#61DAFB" strokeWidth="1" fill="none" transform="rotate(120 12 12)"/>
                </svg>
                React
              </span>
            </p>
          </div>
        </div>
      </footer>

      {/* Modals */}
      {showExperienceForm && (
        <ModalForm onClose={() => { setShowExperienceForm(false); setExperienceError(''); }}>
          <ExperienceForm 
            onClose={() => { setShowExperienceForm(false); setExperienceError(''); }} 
            onAdd={addExperience}
            error={experienceError}
          />
        </ModalForm>
      )}

      {showProjectForm && (
        <ModalForm onClose={() => { setShowProjectForm(false); setProjectError(''); }}>
          <ProjectForm 
            onClose={() => { setShowProjectForm(false); setProjectError(''); }} 
            onAdd={addProject}
            error={projectError}
          />
        </ModalForm>
      )}

      {showLoginForm && (
        <ModalForm onClose={() => { setShowLoginForm(false); setLoginError(''); }}>
          <LoginForm 
            onClose={() => { setShowLoginForm(false); setLoginError(''); }} 
            onAdd={login}
            error={loginError}
          />
        </ModalForm>
      )}
    </div>
  )
}

export default App;
