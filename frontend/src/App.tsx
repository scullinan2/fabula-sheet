import { greet } from 'fabula-rules-engine'
import { useEffect, useState } from 'react'

function App() {
  const [msg, setMsg] = useState('')

  useEffect(() => {
    setMsg(greet())
  }, [])

  return <p>{msg}</p>
}

export default App