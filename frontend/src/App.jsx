import './App.css';
import {useState, useEffect} from 'react'
import Projects from './components/Projects';
import Experiences from './components/Experiences';
import Button from './components/Button';
import ModalForm from './components/ModalForm';
import { ExperienceForm } from './components/ExperienceForm';
import { ProjectForm } from './components/ProjectForm';


function App() {

  const [projects, setprojects] = useState([])
  const [experiences, setexperiences] = useState([])

  const [showExperienceForm, setShowExperienceForm] = useState(false)
  const [showProjectForm, setShowProjectForm] = useState(false)
  const [loggedIn, setLoggedIn] = useState(false)



  useEffect( () => {
    const getProjects = async () => {
      const projectsFromServer = await fetchProjects()
      setprojects(projectsFromServer)
    }

    const getExperiences = async () => {
      const experiencesFromServer = await fetchExperiences()
      setexperiences(experiencesFromServer)
    }

    getProjects()
    getExperiences()
  }, [])


  const fetchProjects = async () => {
    const res = await fetch('http://localhost:8000/projects', {
      credentials: "include"
    }
    )
    const data = await res.json()

    return data
  }


  const fetchExperiences = async () => {
    const res = await fetch('http://localhost:8000/experiences')
    const data = await res.json()

    return data
  }



  const deleteExperience = async (id) => 
  {
    const res = await fetch(`http://localhost:8000/experiences/${id}`, {
      method: 'DELETE',
    })

    res.status === 200
    ? setexperiences(experiences.filter((exp) => exp.id !== id))
    : alert("Failed to delete experience")
  }

  const addExperience = async (experience) => 
  {
    const res = await fetch('http://localhost:8000/experiences', {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-type': 'application/json',
      },
      body: JSON.stringify(experience),
    })

    const data = await res.json()

    setexperiences([...experiences, data])
  }

  const deleteProject = async (id) => 
  {
    const res = await fetch(`http://localhost:8000/projects/${id}`, {
      method: 'DELETE',

    })

    res.status === 200
    ? setprojects(projects.filter((exp) => exp.id !== id))
    : alert("Failed to delete project")
  }

  const addProject = async (project) => 
  {
    const res = await fetch('http://localhost:8000/projects', {
      method: 'POST',
      headers: {
        'Content-type': 'application/json',
      },
      body: JSON.stringify(project),
    })

    const data = await res.json()

    setprojects([...projects, data])
  }

  const toggleExperienceForm = () => {
    setShowExperienceForm(!showExperienceForm)
  }

  const toggleProjectForm = () => {
    setShowProjectForm(!showProjectForm)
  }

  const toggleLoggedIn = () => {
    setLoggedIn(!loggedIn)
  }




  return (
    <div className="App">

      {/* Placeholder Image */}
      <img src='https://www.benjaminlellouch.com/assets/images/profile/profile.png' alt="a broke boy" height="180" width="180" />
      <h1>Benjamin Lellouch</h1>
      <Button color={loggedIn ? "red" :"green"} text={loggedIn ? "Logout" : "Login"} onClick={toggleLoggedIn}/>
      <h2>About me</h2>
      <p>This an about me.</p>
      <p>
        This is a test to see if it gets bigger and stuff but I should probably change the css anyways.
      </p>

      {/* Experience Form pop up */}
      <div>{showExperienceForm ? <ModalForm form={
        <ExperienceForm onClose={toggleExperienceForm} onAdd={addExperience}/>
      }/> : null}
      </div>

      <div>{showProjectForm ? <ModalForm form={
        <ProjectForm onClose={toggleProjectForm} onAdd={addProject}/>
      }/> : null}
      </div>



      {/* Projects */}
      <div>
          <h2>Projects {loggedIn ? <Button text={"Add"} color={"green"} onClick={toggleProjectForm}/> : null}</h2>
          <Projects projects={projects} onDelete={deleteProject} loggedIn={loggedIn}/>
        </div>

       {/* Experience */}
      <div>
          <h2>Experience {loggedIn ? <Button text={"Add"} color={"green"} onClick={toggleExperienceForm}/>: null}</h2>
        <Experiences experiences={experiences} onDelete={deleteExperience} loggedIn={loggedIn}/>
      </div>
        

        
    </div>
  );
}

export default App;
