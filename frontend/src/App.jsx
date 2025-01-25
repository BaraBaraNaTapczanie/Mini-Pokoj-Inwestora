import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { DataTable} from './assets/Stocks/DataTable'

function App() {
  const [loading, setLoading] = useState(false)
  const [data, setData] = useState([])

  useEffect(() => {
    const interval = setInterval(()=>{
      fetch("http://localhost:8080/tabledata")
        .then(response => response.json())
        .then(json => setData(json))
        .finally(()=> {
          setLoading(false)
        })}, 12000)
      return () => clearInterval(interval)
  }, [])

  return (
    <div className="App">
      {loading ? (
        <div>Loading...</div>
      ) : (
        <>
          <h1>Instrumenty</h1>
          <table border={1}>
            <tr>
              <th>Symbol</th>
              <th>Zmiana</th>
              <th>Cena Otwarcia</th>
              <th>Cena Zamkniecia</th>
            </tr>
            {data.map(data => (
              <tr key={data.symbol}>
                <td>{data.symbol}</td>
                <td>{data.open}</td>
                <td>{data.open}</td>
                <td>{data.close}</td>               
              </tr>
            ))}
          </table>
        </>
      )}
    </div>
  )
}

export default App
