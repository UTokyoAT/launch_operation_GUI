import { useState, useEffect, useRef } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import axios from 'axios'
import './App.css'
import type { Log } from './Log'
import LogView from './LogView'
import Button from './Button'
function App() {
  const [logs, setLogs] = useState<Log[]>([]);
  const [acceptNames, setAcceptNames] = useState<string[]>([]);
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    axios.get("http://localhost:8080/accept_names")
      .then((response) => {
        setAcceptNames(response.data)
      })
      .catch((error) => {
        console.error("取得に失敗しました" +error)
      })

    const websocket = new WebSocket("ws://localhost:8080/log")
    socketRef.current = websocket

    const onMessage = (event: MessageEvent<string>) => {
      const log = JSON.parse(event.data)
      setLogs(Object.entries(log).map(([name, value]: [string, any]) => ({ name, value: value.toString() })))
    }

    websocket.addEventListener("message", onMessage)

    return () => {
      websocket.close()
      websocket.removeEventListener("message", onMessage)
    }
  }, [])

  return (
    <>
      <div className="carousel">
        {logs.map((log) => (
          <div className="carousel-item card bg-secondary m-5">
            <LogView key={log.name} {...log} />
          </div>
        ))}
      </div>
      <br />
      {acceptNames.map((name) => (
        <Button name={name} />
      ))}
    </>
  )
}

export default App
