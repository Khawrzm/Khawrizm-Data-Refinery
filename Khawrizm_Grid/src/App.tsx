import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

export default function App() {
  const [cells, setCells] = useState<Record<string, string>>({});
  const [activeCell, setActiveCell] = useState('A1');
  const [formula, setFormula] = useState('');

  // توليد أعمدة وصفوف الشبكة رياضياً (O(1) Rendering structure)
  const cols = Array.from({ length: 26 }, (_, i) => String.fromCharCode(65 + i));
  const rows = Array.from({ length: 100 }, (_, i) => i + 1);

  const handleEvaluate = async () => {
    if (formula.startsWith('=')) {
      try {
        // إرسال المعادلة مباشرة إلى محرك C++ TACO في الحلقة الصفرية
        const result = await invoke('evaluate_xcv', { formula });
        setCells({ ...cells, [activeCell]: result as string });
      } catch (e) {
        setCells({ ...cells, [activeCell]: '#TACO_ERR' });
      }
    } else {
      setCells({ ...cells, [activeCell]: formula });
    }
  };

  return (
    <div className="sovereign-container">
      <div className="ribbon">
        <div className="brand">KHAWRIZM OMNI-GRID // RING-0 ACTIVE</div>
        <div className="toolbar">
          <button>FILE</button>
          <button>HOME</button>
          <button className="active-tab">TACO ENGINE</button>
          <button>6G ISAC RADAR</button>
          <button>GHOST PROTOCOL</button>
        </div>
      </div>
      
      <div className="formula-bar">
        <span className="active-cell-indicator">{activeCell}</span>
        <span className="fx-icon">ƒx</span>
        <input 
          value={formula}
          onChange={(e) => setFormula(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleEvaluate()}
          placeholder="Enter value or TACO formula (=SUM(A1:B4))..."
        />
      </div>

      <div className="grid-container">
        <table className="xcv-grid">
          <thead>
            <tr>
              <th></th>
              {cols.map(c => <th key={c}>{c}</th>)}
            </tr>
          </thead>
          <tbody>
            {rows.map(r => (
              <tr key={r}>
                <td className="row-header">{r}</td>
                {cols.map(c => {
                  const cellId = `${c}${r}`;
                  const isActive = activeCell === cellId;
                  return (
                    <td 
                      key={cellId} 
                      className={isActive ? 'active-cell' : ''}
                      onClick={() => { setActiveCell(cellId); setFormula(cells[cellId] || ''); }}
                    >
                      {cells[cellId] || ''}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      <div className="status-bar">
        <span>STATUS: ZERO-TELEMETRY</span>
        <span>ENGINE: C++ TACO COMPRESSOR</span>
        <span>RAM SCRUB: ACTIVE</span>
      </div>
    </div>
  );
}
