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

      <form>
        <div className="mt-2">
          <div className="flex items-center rounded-md bg-white pl-3 outline-1 -outline-offset-1 outline-gray-300 has-[input:focus-within]:outline-2 has-[input:focus-within]:-outline-offset-2 has-[input:focus-within]:outline-indigo-600">


          <input type="text" id="taskbox" name="taskbox_name" placeholder="Enter todo" className="block min-w-0 grow py-1.5 pr-3 pl-1 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6"></input>
          <button type="button" className="oou oow ooz ope opk opl opm opp opr ops opt">Add</button>
          </div>
        </div>
      </form>
    </>
  )
}

export default App
