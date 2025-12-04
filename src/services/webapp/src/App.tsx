import { Button } from '@/components/ui/button'
import './App.css'
import { ModeToggle } from './components/mode-toggle'

function App() {

  return (
    <div className="flex min-h-svh flex-col items-center justify-center">
      <Button>Click me</Button>
      <ModeToggle/>
    </div>

  )
}

export default App
