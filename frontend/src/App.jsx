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

    getProjects()
  }, [])


  const fetchProjects = async () => {
    const res = await fetch('http://localhost:8000/projects')
    const data = await res.json()
    console.log(data)

    return data
  }


  const deleteExperience = (id) => 
  {
    setexperiences(experiences.filter((exp) => exp.id !== id))
  }

  const addExperience = (experience) => 
  {
    const id = Math.floor(Math.random() * 10000) + 1
    const newExperience = {id, ...experience}
    setexperiences([...experiences, newExperience])
  }

  const deleteProject = (id) => 
  {
    setprojects(projects.filter((exp) => exp.id !== id))
  }

  const addProject = (project) => 
  {
    const id = Math.floor(Math.random() * 10000) + 1
    const newProject = {id, ...project}
    setprojects([...projects, newProject])
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
      <Button color={"green"} text={"Login"} onClick={toggleLoggedIn}/>
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
