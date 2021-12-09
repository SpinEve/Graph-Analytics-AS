import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

if __name__ == '__main__':

    data_name = input('Data name:')
    df = pd.read_csv('../output/' + data_name + '.csv')

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Average Degree')
    ax.set_ylim([0, df['prod'].max()+1])
    ax.set_title('Estimate average degree on ' + data_name + ' data')
    plt.plot(df['p'], df['avgd'], '-ko', label='Ground Truth')
    plt.plot(df['p'], df['orid'], '-go', label='Smooth Estimator')
    plt.plot(df['p'], df['prod'], '-ro', label='Proposed Estimator')
    plt.legend()
    plt.savefig(data_name+'_avgd.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Average Clustering Coefficient')
    ax.set_title('Estimate average clustering coefficient on ' +
                 data_name + ' data')
    ax.set_ylim([0, df['proavgc'].max()+0.1])
    plt.plot(df['p'], df['avgc'], '-ko', label='Ground Truth')
    plt.plot(df['p'], df['oriavgc'], '-go', label='Original Estimator')
    plt.plot(df['p'], df['proavgc'], '-ro', label='Proposed Estimator')
    plt.legend()
    plt.savefig(data_name+'_avgc.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Error')
    ax.set_title('Error of average clustering coefficient on ' +
                 data_name + ' data')
    plt.plot(df['p'], (1-df['oriavgc']/df['avgc'])**2,
             '-go', label='Original Estimator Error')
    plt.plot(df['p'], (1-df['proavgc']/df['avgc'])**2,
             '-ro', label='Proposed Estimator Error')
    plt.legend()
    plt.savefig(data_name+'_avgcerror.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Error')
    ax.set_title('Error of estimate average degree on ' + data_name + ' data')
    plt.plot(df['p'], (1-df['orid']/df['avgd'])**2,
             '-go', label='Smooth Estimator Error')
    plt.plot(df['p'], (1-df['prod']/df['avgd'])**2,
             '-ro', label='Proposed Estimator Error')
    plt.legend()
    plt.savefig(data_name+'_avgderror.jpg', dpi=450)
