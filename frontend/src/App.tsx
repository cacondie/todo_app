import { useEffect, useState } from "react"

function App() {
  const [status, setStatus] = useState("loading")

  useEffect(() => {
    fetch("http://localhost:3000/api/health")
    .then(res => res.json())
    .then(data => setStatus(data.status))
  }, [])

  return (
    <>
      <h1>Rust + React</h1>
      <p>API status: {status}</p>
    </>
  )
}

export default App
