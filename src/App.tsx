import { useEffect, useRef } from 'react';
import init, { Simulation, Body } from '../physics-engine/pkg/physics_engine';

export default function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const zoomRef = useRef<number>(1.0);

  useEffect(() => {
    let animationFrameId: number;
    let handleWheel: (e: WheelEvent) => void;
    let canvasElement: HTMLCanvasElement;

    const runSimulation = async () => {
      // Initialize WASM
      const wasm = await init();
      
      // Set Initial state
      const sim = new Simulation();
      // Heavy central star (Mass: 100 Trillion)
      sim.add_body(new Body(100000000000000.0, 400.0, 300.0, 0.0, 0.0, 25.0));
      
      // Inner orbiting planet (Distance: 150 pixels, Balanced Velocity: 6.67)
      sim.add_body(new Body(1.0, 400.0, 150.0, 6.67, 0.0, 8.0));

      // Outer orbiting planet (Distance: 250 pixels, Balanced Velocity: 5.16)
      sim.add_body(new Body(1.0, 400.0, 50.0, 5.16, 0.0, 5.0));

      const canvas = canvasRef.current;
      if (!canvas) return;
      canvasElement = canvas;
      
      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      handleWheel = (e: WheelEvent) => {
        e.preventDefault();
        zoomRef.current += e.deltaY * -0.001;
        zoomRef.current = Math.min(Math.max(0.1, zoomRef.current), 5.0);
      };
      canvas.addEventListener('wheel', handleWheel);

      // Render Loop
      const renderLoop = () => {
        sim.tick(0.1); // Advance 

        // Clear the previous frame
        ctx.fillStyle = "#0f172a"; 
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        ctx.save();
        ctx.translate(400, 300);
        ctx.scale(zoomRef.current, zoomRef.current);
        ctx.translate(-400, -300);

        const bodiesPtr = sim.bodies_ptr();
        const numBodies = sim.num_bodies();
        
        const bodiesArray = new Float64Array(wasm.memory.buffer, bodiesPtr, numBodies * 8);

        ctx.fillStyle = "#38bdf8"; 
        
        for (let i = 0; i < numBodies; i++) {
            const x = bodiesArray[i * 8 + 1];
            const y = bodiesArray[i * 8 + 2];
            const radius = bodiesArray[i * 8 + 7];

            ctx.beginPath();
            ctx.arc(x, y, radius, 0, Math.PI * 2);
            ctx.fill();
        }

        ctx.restore();
        animationFrameId = requestAnimationFrame(renderLoop);
      };

      animationFrameId = requestAnimationFrame(renderLoop);
    };

    runSimulation();

    return () => {
      if (canvasElement && handleWheel) {
        canvasElement.removeEventListener('wheel', handleWheel);
      }
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return (
    <div className="flex items-center justify-center h-screen bg-slate-900 overflow-hidden">
      <canvas 
        ref={canvasRef} 
        width={800} 
        height={600} 
        className="border border-slate-700 rounded-lg shadow-2xl"
      />
    </div>
  );
}