import React, { useState, useEffect } from 'react'
import ItemList from './components/ItemList'
import Button from './components/Button'
import ModalForm from './components/ModalForm'
import DarkModeToggle from './components/DarkModeToggle'
import { ExperienceForm } from './components/ExperienceForm'
import { ProjectForm } from './components/ProjectForm'
import { LoginForm } from './components/LoginForm'
import { useDarkMode } from './hooks/useDarkMode'

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
      const projectsFromServer = await fetchProjects()
      setProjects(projectsFromServer)
    } catch (error) {
      console.error('Failed to load projects:', error)
    }
  }

  const loadExperiences = async () => {
    try {
      const experiencesFromServer = await fetchExperiences()
      setExperiences(experiencesFromServer)
    } catch (error) {
      console.error('Failed to load experiences:', error)
    }
  }

  const attemptAutoLogin = async () => {
    try {
      const success = await alreadyLoggedIn()
      if (success) {
        setLoggedIn(true)
      }
    } catch (error) {
      console.error('Auto-login failed:', error)
    }
  }

  const fetchProjects = async () => {
    const res = await fetch('http://localhost:8000/projects', {
      credentials: 'include'
    })
    return await res.json()
  }

  const fetchExperiences = async () => {
    const res = await fetch('http://localhost:8000/experiences')
    return await res.json()
  }

  const alreadyLoggedIn = async () => {
    const res = await fetch('http://localhost:8000/login', {
      credentials: 'include',
    })
    const data = await res.json()
    return data.success
  }

  // Experience handlers
  const deleteExperience = async (id) => {
    try {
      const res = await fetch(`http://localhost:8000/experiences/${id}`, {
        method: 'DELETE',
        credentials: 'include'
      })

      if (res.status === 200) {
        setExperiences(experiences.filter((exp) => exp.id !== id))
      } else {
        alert('Failed to delete experience')
      }
    } catch (error) {
      alert('Failed to delete experience')
    }
  }

  const addExperience = async (experienceData) => {
    try {
      setExperienceError('') // Clear any previous errors
      const { clearForm, ...experience } = experienceData
      
      const res = await fetch('http://localhost:8000/experiences', {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-type': 'application/json',
        },
        body: JSON.stringify(experience),
      })

      if (res.status === 200) {
        const data = await res.json()
        setExperiences([...experiences, data])
        setExperienceError('')
        setShowExperienceForm(false)
        clearForm()
      } else {
        setExperienceError('Failed to add experience. Please try again.')
      }
    } catch (error) {
      setExperienceError('Failed to add experience. Please check your connection.')
    }
  }

  // Project handlers
  const deleteProject = async (id) => {
    try {
      const res = await fetch(`http://localhost:8000/projects/${id}`, {
        method: 'DELETE',
        credentials: 'include'
      })

      if (res.status === 200) {
        setProjects(projects.filter((project) => project.id !== id))
      } else {
        alert('Failed to delete project')
      }
    } catch (error) {
      alert('Failed to delete project')
    }
  }

  const addProject = async (projectData) => {
    try {
      setProjectError('') // Clear any previous errors
      const { clearForm, ...project } = projectData
      
      const res = await fetch('http://localhost:8000/projects', {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-type': 'application/json',
        },
        body: JSON.stringify(project),
      })

      if (res.status === 200) {
        const data = await res.json()
        setProjects([...projects, data])
        setProjectError('')
        setShowProjectForm(false)
        clearForm()
      } else {
        setProjectError('Failed to add project. Please try again.')
      }
    } catch (error) {
      setProjectError('Failed to add project. Please check your connection.')
    }
  }

  // Auth handlers
  const login = async (credentials) => {
    try {
      setLoginError('') // Clear any previous errors
      const res = await fetch('http://localhost:8000/login', {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-type': 'application/json',
        },
        body: JSON.stringify(credentials),
      })

      const data = await res.json()

      if (data.success) {
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
      const res = await fetch('http://localhost:8000/logout', {
        method: 'POST',
        credentials: 'include'
      })

      if (res.status === 200) {
        setLoggedIn(false)
      } else {
        alert('Failed to logout')
      }
    } catch (error) {
      alert('Failed to logout')
    }
  }

  const handleLoginClick = async () => {
    const alreadyAuthenticated = await alreadyLoggedIn()
    if (alreadyAuthenticated) {
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
              src="https://media.licdn.com/dms/image/v2/D4E03AQFdnuZAAnTQ_A/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1663869957381?e=1761782400&v=beta&t=RBK2wAqWgo_XoDQr5RDRUH30AkVCSmsjnDvy5-FgG54" 
              alt="Benjamin Lellouch" 
              className="w-32 h-32 rounded-full mx-auto mb-4 object-cover shadow-lg"
            />
            <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors duration-300">Benjamin Lellouch</h1>
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
