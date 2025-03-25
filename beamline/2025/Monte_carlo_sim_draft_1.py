//Made Using Help from Claude for optimization

import numpy as np
import matplotlib.pyplot as plt
from scipy.constants import c, h, pi
from typing import Tuple, List

class PositronMonteCarlo:
    def __init__(self, seed=42):
        """
        Initialize Monte Carlo simulation parameters
        """
        np.random.seed(seed)
        
        # Beam Parameters
        self.beam_energy = 1.5  # GeV
        self.beam_radius = 0.01  # 1 cm
        self.num_particles = 10000  # Number of simulated positrons
        
        # Detector Geometry
        self.hodoscope_layers = 4
        self.fiber_diameter = 0.5e-3  # 0.5 mm
        self.fiber_length = 0.15  # 15 cm
        
        # Material Properties
        self.quartz_refractive_index = 1.46
        self.quartz_radiation_length = 0.12  # 12 cm
        
        # SiPM Properties
        self.sipm_efficiency = 0.35
        self.sipm_dark_count = 100  # Hz
        self.sipm_gain = 1e6
        
        # Magnetic Filter
        self.magnetic_field_strength = 0.5  # Tesla
        
        # Simulation Results
        self.trajectories = []
        self.cherenkov_photons = []
        self.energy_depositions = []
    
    def generate_beam(self) -> np.ndarray:
        """
        Generate initial positron beam with Gaussian distribution
        
        Returns:
        numpy array of initial positron positions and momenta
        """
        # Generate beam with Gaussian spatial distribution
        x = np.random.normal(0, self.beam_radius/3, self.num_particles)
        y = np.random.normal(0, self.beam_radius/3, self.num_particles)
        z = np.zeros(self.num_particles)
        
        # Initial momentum (all along z-axis)
        px = np.zeros(self.num_particles)
        py = np.zeros(self.num_particles)
        pz = np.full(self.num_particles, self.beam_energy)
        
        return np.column_stack([x, y, z, px, py, pz])
    
    def apply_magnetic_filter(self, particles: np.ndarray) -> np.ndarray:
        """
        Apply magnetic field to filter and deflect positrons
        
        Args:
        particles: numpy array of particle states
        
        Returns:
        filtered particles
        """
        # Lorentz force calculation
        charge = 1  # Positron charge
        magnetic_deflection_angle = (charge * self.magnetic_field_strength) / (self.beam_energy * 1e9)
        
        # Rotate momentum vector
        for i in range(len(particles)):
            rotation_matrix = np.array([
                [np.cos(magnetic_deflection_angle), -np.sin(magnetic_deflection_angle), 0],
                [np.sin(magnetic_deflection_angle), np.cos(magnetic_deflection_angle), 0],
                [0, 0, 1]
            ])
            particles[i, 3:6] = np.dot(rotation_matrix, particles[i, 3:6])
        
        return particles
    
    def multiple_scattering(self, particles: np.ndarray, foil_thickness: float = 0.5e-3) -> np.ndarray:
        """
        Simulate multiple Coulomb scattering in Al foil
        
        Args:
        particles: numpy array of particle states
        foil_thickness: foil thickness in meters
        
        Returns:
        scattered particles
        """
        for i in range(len(particles)):
            # Highland formula for approximating scattering angle
            theta_0 = 13.6e-3 / (self.beam_energy * 1e9) * np.sqrt(foil_thickness / (9.37 * 0.027))
            
            # Random angular deflection
            theta = np.random.normal(0, theta_0)
            phi = np.random.uniform(0, 2*pi)
            
            # Apply rotation
            Rx = np.array([
                [1, 0, 0],
                [0, np.cos(theta), -np.sin(theta)],
                [0, np.sin(theta), np.cos(theta)]
            ])
            
            Rz = np.array([
                [np.cos(phi), -np.sin(phi), 0],
                [np.sin(phi), np.cos(phi), 0],
                [0, 0, 1]
            ])
            
            momentum = particles[i, 3:6]
            new_momentum = np.dot(Rz, np.dot(Rx, momentum))
            particles[i, 3:6] = new_momentum
        
        return particles
    
    def cherenkov_radiation(self, particles: np.ndarray) -> List[dict]:
        """
        Simulate Cherenkov radiation generation in quartz fibers
        
        Args:
        particles: numpy array of particle states
        
        Returns:
        List of Cherenkov photon events
        """
        cherenkov_events = []
        
        for particle in particles:
            velocity = np.linalg.norm(particle[3:6]) / self.beam_energy
            cos_theta_c = 1 / (velocity * self.quartz_refractive_index)
            
            if cos_theta_c <= 1:
                # Cherenkov threshold condition met
                num_photons = self.calculate_photon_yield(velocity)
                
                cherenkov_event = {
                    'position': particle[:3],
                    'momentum': particle[3:6],
                    'num_photons': num_photons,
                    'wavelength_range': (200e-9, 600e-9)  # UV to visible
                }
                cherenkov_events.append(cherenkov_event)
        
        return cherenkov_events
    
    def calculate_photon_yield(self, velocity: float) -> int:
        """
        Calculate number of Cherenkov photons using Frank-Tamm formula
        
        Args:
        velocity: particle velocity
        
        Returns:
        Number of photons
        """
        def integrand(wavelength):
            return wavelength**-2
        
        # Simplified photon yield calculation
        photon_yield = int(100 * (1 - 1/(velocity**2 * self.quartz_refractive_index**2)))
        return max(0, photon_yield)
    
    def simulate(self):
        """
        Run full Monte Carlo simulation
        """
        # Initial beam generation
        particles = self.generate_beam()
        
        # Apply magnetic filter
        particles = self.apply_magnetic_filter(particles)
        
        # Multiple scattering in Al foil
        particles = self.multiple_scattering(particles)
        
        # Cherenkov radiation generation
        self.cherenkov_photons = self.cherenkov_radiation(particles)
        
        # Store trajectories
        self.trajectories = particles
    
    def visualize_results(self):
        """
        Visualize simulation results
        """
        plt.figure(figsize=(15, 10))
        
        # Trajectory Plot
        plt.subplot(2, 2, 1)
        trajs = self.trajectories
        plt.scatter(trajs[:, 0], trajs[:, 2], alpha=0.1)
        plt.title('Positron Trajectories')
        plt.xlabel('X Position (m)')
        plt.ylabel('Z Position (m)')
        
        # Cherenkov Photon Distribution
        plt.subplot(2, 2, 2)
        photon_counts = [event['num_photons'] for event in self.cherenkov_photons]
        plt.hist(photon_counts, bins=30)
        plt.title('Cherenkov Photon Yield')
        plt.xlabel('Number of Photons')
        plt.ylabel('Frequency')
        
        plt.tight_layout()
        plt.show()

# Run Simulation
np.random.seed(42)
sim = PositronMonteCarlo()
sim.simulate()
sim.visualize_results()
