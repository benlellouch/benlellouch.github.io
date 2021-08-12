import './App.css';
import {useState} from 'react'
import Projects from './components/Projects';
import Experiences from './components/Experiences';

function App() {

  const [projects, setprojects] = useState([
    {
      id:1,
      title:"This website",
      description:"This is my first website built with React",
      link:"https://github.com/benlellouch/benlellouch.github.io"
    },
    {
      id:2,
      title:"Cool project",
      description:"This is a cool project",
      link:"https://github.com/benlellouch/benlellouch.github.io"
    },
    
  ])

  const [experiences, setexperiences] = useState([
    {
      id:1,
      title:"Software Engineer",
      company:"J.P. Morgan",
      year: "jun 2021 - aug 2021",
      description:"This is my first website built with React",
      org_link:"https://github.com/benlellouch/benlellouch.github.io"
    },
    {
      id:2,
      title:"Software Engineer",
      year: "jun 2020 - aug 2020",
      company:"Raymarine",
      description:"Another Internship my G",
      org_link:"https://github.com/benlellouch/benlellouch.github.io"
    },
  ])

  return (
    <div className="App">

        {/* Placeholder Image */}
        <img src='https://www.benjaminlellouch.com/assets/images/profile/profile.png' alt="a broke boy" height="180" width="180" />
        <h1>Benjamin Lellouch</h1>
        <h2>About me</h2>
        <p>This an about me.</p>
        <p>
          This is a test to see if it gets bigger and stuff but I should probably change the css anyways.
        </p>

        <Projects projects={projects}/>

        <Experiences experiences={experiences}/>

    </div>
  );
}

export default App;
