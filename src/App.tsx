import { Fit } from '@/components/Fit/Fit';
import { Footer } from '@/components/Footer/Footer';
import { Hero } from '@/components/Hero/Hero';
import { Navbar } from '@/components/Navbar/Navbar';
import { Patterns } from '@/components/Patterns/Patterns';
import { Usage } from '@/components/Usage/Usage';
import { page } from '@/components/shared/shared.css';

export default function App() {
  return (
    <div className={page}>
      <Navbar />
      <main id="top">
        <Hero />
        <Fit />
        <Patterns />
        <Usage />
      </main>
      <Footer />
    </div>
  );
}
