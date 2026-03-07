import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import ZinkIDE from './ZinkIDE.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ZinkIDE />
  </StrictMode>,
)
